package handlers

import (
	"github.com/george/web/pkg/config"
	"github.com/george/web/pkg/models"
	"github.com/george/web/pkg/render"
	"net/http"
)

var Repo *Repository

type Repository struct {
	App *config.AppConfig
}

// NewRepo returns a new Repository that uses the provided AppConfig.
//
// It creates a new Repository and assigns the provided AppConfig to its App
// field. The returned Repository is ready for use with the NewHandlers
// function.
func NewRepo(a *config.AppConfig) *Repository {
	return &Repository{
		App: a,
	}
}

// NewHandlers sets the Repository used by the handlers.
//
// It takes a pointer to a Repository as an argument and assigns it to the
// package-level Repo variable. This variable is used by all the handlers
// to access application-wide configuration settings.
func NewHandlers(r *Repository) {
	Repo = r
}

// Home handles HTTP requests for the home page.
//
// It renders the 'home.page.html' template in the 'templates' directory and
// writes the result to the given ResponseWriter.
func (m *Repository) Home(w http.ResponseWriter, r *http.Request) {
	render.RenderTemplate(w, "home.page.html", &models.TemplateData{})
}

// About handles HTTP requests for the about page.
//
// It renders the 'about.page.html' template in the 'templates' directory and
// writes the result to the given ResponseWriter.
func (m *Repository) About(w http.ResponseWriter, r *http.Request) {
	render.RenderTemplate(w, "about.page.html", &models.TemplateData{})
}
