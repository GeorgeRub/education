package main

import (
	"github.com/george/web/pkg/config"
	"github.com/george/web/pkg/handlers"
	"github.com/george/web/pkg/render"
	"log"
	"net/http"
)

// main is the entry point for the application.
//
// It sets up the handlers for the home and about pages, starts the server,
// and logs any errors that might occur.
func main() {

	var app config.AppConfig

	tc, err := render.CreateTemplateCache()
	if err != nil {
		log.Fatal(err)
	}

	app.TemplateCache = tc
	app.UseCache = false
	app.Port = ":8080"

	repo := handlers.NewRepo(&app)
	handlers.Repo = repo

	render.NewTemplates(&app)

	http.HandleFunc("/home", handlers.Repo.Home)
	http.HandleFunc("/about", handlers.Repo.About)

	log.Println("Server started on port http://localhost" + app.Port + "/home")

	err = http.ListenAndServe(app.Port, nil)
	if err != nil {
		log.Fatal(err)
	}

}
