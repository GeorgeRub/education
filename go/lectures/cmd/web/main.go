package main

import (
	"github.com/george/web/pkg/handlers"
	"log"
	"net/http"
)

const port = ":8080"

// main is the entry point for the application.
//
// It sets up the handlers for the home and about pages, starts the server,
// and logs any errors that might occur.
func main() {

	http.HandleFunc("/", handlers.Home)
	http.HandleFunc("/about", handlers.About)

	log.Println("Server started on port http://localhost" + port)

	err := http.ListenAndServe(port, nil)
	if err != nil {
		log.Fatal(err)
	}

}
