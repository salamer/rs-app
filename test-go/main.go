// package main

// import (
// 	"encoding/base64"
// 	"fmt"
// 	"io"
// 	"net/http"
// 	"strings"

// 	"github.com/golang/snappy"
// )

// func call() {
// 	url := "http://127.0.0.1:8080/echo"

// 	var jsonStr = []byte(`{"title":"Buy cheese and bread for breakfast."}`)

// 	encoded := snappy.Encode(nil, jsonStr)

// 	sEnc := base64.RawStdEncoding.EncodeToString([]byte(encoded))

// 	// req, _ := http.NewRequest("POST", url, bytes.NewBuffer(b.Bytes()))
// 	req, _ := http.NewRequest("POST", url, strings.NewReader(sEnc))

// 	client := &http.Client{}
// 	resp, err := client.Do(req)
// 	if err != nil {
// 		panic(err)
// 	}
// 	defer resp.Body.Close()

// 	fmt.Println("response Status:", resp.Status)
// 	fmt.Println("response Headers:", resp.Header)
// 	body, _ := io.ReadAll(resp.Body)
// 	fmt.Println("response Body:", string(body))
// }

// func main() {
// 	call()
// }

package main

import (
	"log"
	"math/rand"
	"net/http"

	"github.com/gin-contrib/gzip"
	"github.com/gin-gonic/gin"
)

func makeRandomString(size int) string {
	var result []byte
	for i := 0; i < size; i++ {
		result = append(result, byte(65+rand.Intn(25)))
	}
	return string(result)
}

func main() {
	r := gin.Default()
	r.Use(gzip.Gzip(gzip.DefaultCompression))
	r.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK,
			// makeRandomString(10000),
			"Hello World",
		)
	})

	// Listen and Server in 0.0.0.0:8080
	if err := r.Run(":8090"); err != nil {
		log.Fatal(err)
	}
}
