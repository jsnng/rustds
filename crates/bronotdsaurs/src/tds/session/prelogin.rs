use crate::tds::prelude::*;
use crate::tds::session::prelude::*;

#[cfg(feature = "std")]
use tracing::debug;

impl<T: AsyncTransport, O: Observer<Event>> AsyncReceiver<T> for Session<InitialState, T, O> {
    type Error = SessionError;
    type Output<'a>
        =  PreLoginSpan<'a> where Self: 'a;

    async fn receive(&mut self) -> Result<(), Self::Error> {
        self.buffer.reset();

        // read the TDS header
        let mut head = 0;
        while head < PreLoginHeader::LENGTH {
            let n = self
                .stream
                .read(&mut self.buffer.writeable()[..PreLoginHeader::LENGTH])
                .await
                .map_err(|_| SessionError::transport_read_error())?;
            if n == 0 {
                return Err(SessionError::ServerClosedTransportConnection);
            }
            head += n;
        }
        // advance tail past the header
        self.buffer.tail(PreLoginHeader::LENGTH)?;

        let span = PreLoginSpan::new(self.buffer.readable())?;
        let length = span.header().length() as usize;

        if length < PreLoginHeader::LENGTH {
            return Err(DecodeError::invalid_length(format!("PreLogin response: packet length {} less than header length {}", length, PreLoginHeader::LENGTH)).into());
        }

        let payload_length = length - PreLoginHeader::LENGTH;
        let mut reading = 0;
        while reading < payload_length {
            let n = self
                .stream
                .read(&mut self.buffer.writeable()[reading..payload_length])
                .await
                .map_err(|_| SessionError::transport_read_error())?;
            if n == 0 {
                return Err(SessionError::ServerClosedTransportConnection);
            }
            reading += n;
        }
        self.buffer.tail(payload_length)?;

        self.notify(Event::BytesReceived {
            heading: "PreLogin",
            len: self.buffer.readable().len(),
        });
        Ok(())
    }
    fn output(&self) -> Result<Self::Output<'_>, Self::Error> {
        PreLoginSpan::new(self.buffer.readable()).map_err(SessionError::from)
    }
}

#[derive(Debug)]
pub enum InitialStateTransition<T, O> {
    #[cfg(feature = "tls")]
    TlsSslNegotiation(Session<TlsSslNegotiationState, T, O>),
    LoginReady(Session<LoginReadyState, T, O>),
    #[cfg(feature = "tds8.0")]
    TlsNegotiation(Session<TlsNegotiationState, T, O>),
}

/// TODO: *vomit*
/// dual AsyncTransport + Transport bound is the cost of TLS + transport agnostic design.
#[cfg(not(feature = "tds8.0"))]
impl<T: AsyncTransport + Transport, O: Observer<Event>> Session<InitialState, T, O> {
    pub async fn transition(
        mut self,
        prelogin: PreLoginPacket,
    ) -> Result<InitialStateTransition<T, O>, SessionError> {
        AsyncTransport::set_read_timeout(&mut self.stream, self.timers.connection)
            .map_err(|_| SessionError::transport_read_error())?;
        AsyncTransport::set_write_timeout(&mut self.stream, self.timers.connection)
            .map_err(|_| SessionError::transport_write_error())?;
        self.send(prelogin).await?;
        self.notify(Event::PreLoginSent);

        self.receive().await?;

        let bytes = PreLoginSpan::populate(self.buffer.readable())?
            .encryption()
            .unwrap_or(PreLoginEncryptionOptions::NotSupported as u8);
        self.notify(Event::PreLoginReceived);

        #[cfg(feature = "std")]
        debug!("Server encryption byte = 0x{:02x}", bytes);

        let encryption: PreLoginEncryptionOptions = bytes
            .try_into()?;

        #[cfg(feature = "std")]
        debug!("Parsed as {:?}", encryption);

        match encryption {
            PreLoginEncryptionOptions::Off | PreLoginEncryptionOptions::NotSupported => {
                self.notify(Event::StateTransition {
                    from: "Initial",
                    to: "LoginReady",
                });
                Ok(InitialStateTransition::LoginReady(Session {
                    stream: self.stream,
                    observer: self.observer,
                    buffer: self.buffer,
                    timers: self.timers,
                    state: LoginReadyState,
                }))
            }
            PreLoginEncryptionOptions::On | PreLoginEncryptionOptions::Required => {
                #[cfg(feature = "tls")]
                {
                    self.notify(Event::StateTransition {
                        from: "Initial",
                        to: "TlsSslNegotiation",
                    });
                    Ok(InitialStateTransition::TlsSslNegotiation(Session {
                        stream: self.stream,
                        observer: self.observer,
                        buffer: self.buffer,
                        timers: self.timers,
                        state: TlsSslNegotiationState,
                    }))
                }
                #[cfg(not(feature = "tls"))]
                Err(SessionError::Unimplemented)
            }
            _ => Err(DecodeError::invalid_field(format!("PreLogin response: unsupported encryption option: {:?}", encryption)).into()),
        }
    }
}

#[cfg(feature = "tds8.0")]
impl<T: AsyncTransport, O: Observer<Event>> Session<InitialState, T, O> {
    pub fn transition(self) -> Result<InitialStateTransition<T, O>, SessionError> {
        todo!()
    }
}

#[cfg(feature = "tls")]
impl<T: AsyncTransport + Transport, O: Observer<Event>> Session<TlsSslNegotiationState, T, O> {
    pub async fn transition<P, H, F>(
        self,
        server_name: &str,
        handshaker: H,
        factory: F,
    ) -> Result<Session<LoginReadyState, P, O>, SessionError>
    where
        P: AsyncTransport,
        H: TlsHandshaker,
        H::HandshakeError: core::fmt::Debug,
        <T as Transport>::Error: core::fmt::Debug,
        F: FnOnce(T, H::Connection) -> P,
    {
        let Session {
            mut stream,
            mut observer,
            timers,
            buffer,
            ..
        } = self;

        let connection = {
            let mut adaptor = TransportAdaptor {
                transport: &mut stream,
                reader: TransportAdaptorBuffer::default(),
                writer: TransportAdaptorBuffer::default(),
            };
            handshaker.handshake(server_name, &mut adaptor).map_err(|e| {
                SessionError::MappedError(alloc::format!("TLS handshake failed {:?}", e))
            })?
        };

        observer.on(&Event::StateTransition {
            from: "TlsSslNegotiation",
            to: "LoginReady",
        });
        Ok(Session {
            stream: factory(stream, connection),
            observer,
            timers,
            buffer,
            state: LoginReadyState,
        })
    }
}
