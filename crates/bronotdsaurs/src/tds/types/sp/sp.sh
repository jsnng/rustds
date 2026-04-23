#! /bin/zsh

arr=("SpCursor" "SpCursorOpen" "SpCursorPrepare" "SpCursorExecute" "SpCursorPrepExec" "SpCursorUnprepare" "SpCursorFetch" "SpCursorOption" "SpCursorClose" "SpExecuteSql" "SpPrepare" "SpExecute" "SpPrepExec" "SpPrepExecRpc" "SpUnprepare")
touch prelude.rs
echo "pub(crate) use crate::tds::types::rpc::ProcId;" > prelude.rs
echo "pub(crate) use crate::tds::types::sp::error::StoredProcedureError;" > prelude.rs
touch mod.rs
echo "pub mod prelude;" > mod.rs
echo "pub mod error;" > mod.rs  
for elm in $arr; do
    touch "sp_${${elm#Sp}:l}.rs"
    echo "pub mod sp_${${elm#Sp}:l};" >> mod.rs
    echo "pub use crate::tds::types::sp::sp_${${elm#Sp}:l};" >> prelude.rs
done
