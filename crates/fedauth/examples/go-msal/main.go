package main

/*
#include <stdlib.h>
#include <stddef.h>
*/
import "C"
import (
	"context"
	"os"
	"unsafe"

	"github.com/AzureAD/microsoft-authentication-library-for-go/apps/confidential"
	"github.com/joho/godotenv"
)

func init() {
	_ = godotenv.Load() // ignore if mising .env
}

//export acquire
func acquire(stsURL *C.uint8_t, stsURLLen C.size_t, spn *C.uint8_t, spnLen C.size_t, nonce *C.uint8_t, nonceLen C.size_t, outLen *C.size_t) *C.uint8_t {
	sts := C.GoStringN((*C.char)(unsafe.Pointer(stsURL)), C.int(stsURLLen))
	scope := C.GoStringN((*C.char)(unsafe.Pointer(spn)), C.int(spnLen))
	_ = nonce
	_ = nonceLen

	credential, err := confidential.NewCredFromSecret(os.Getenv("AZUREAD_CLIENT_SECRET"))
	if err != nil {
		return nil
	}

	client, err := confidential.New(sts, os.Getenv("AZUREAD_APPLICATION_ID"), credential)
	if err != nil {
		return nil
	}

	result, err := client.AcquireTokenByCredential(context.Background(), []string{scope})
	if err != nil {
		return nil
	}
	token := []byte(result.AccessToken)
	*outLen = C.size_t(len(token))
	ptr := C.CBytes(token)
	return (*C.uint8_t)(ptr)
}

func main() {}
