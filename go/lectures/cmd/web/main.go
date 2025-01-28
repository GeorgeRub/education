package main

import (
	"github.com/alexedwards/scs/v2"
	"github.com/george/web/pkg/config"
	"github.com/george/web/pkg/handlers"
	"github.com/george/web/pkg/render"
	"log"
	"net/http"
	"time"
)

var app config.AppConfig
var session *scs.SessionManager

// main is the entry point for the program.
//
// It sets up the application configuration, connects to the database, and
// starts the web server.
func main() {

	app.InProduction = false

	session = scs.New()
	session.Lifetime = 24 * time.Hour
	session.Cookie.Persist = false
	session.Cookie.SameSite = http.SameSiteLaxMode
	session.Cookie.Secure = false

	app.Session = session

	tc, err := render.CreateTemplateCache()
	if err != nil {
		log.Fatal(err)
	}

	app.TemplateCache = tc
	app.UseCache = false
	app.Port = ":8080"

	repo := handlers.NewRepo(&app)
	//handlers.Repo = repo
	handlers.NewHandlers(repo)

	render.NewTemplates(&app)

	log.Println("Server started on port http://localhost" + app.Port + "/home")

	srv := &http.Server{
		Addr:    app.Port,
		Handler: routes(&app),
	}

	err = srv.ListenAndServe()
	if err != nil {
		log.Fatal(err)
	}

}
