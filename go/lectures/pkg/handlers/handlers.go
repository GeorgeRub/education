package handlers

import (
	"github.com/george/web/pkg/render"
	"net/http"
)

// Home handles HTTP requests for the home page.
//
// It renders the 'home.page.html' template in the 'templates' directory and
// writes the result to the given ResponseWriter.
func Home(w http.ResponseWriter, r *http.Request) {
	render.RenderTemplate(w, "home.page")
}

// About handles HTTP requests for the about page.
//
// It renders the 'about.page.html' template in the 'templates' directory and
// writes the result to the given ResponseWriter.
func About(w http.ResponseWriter, r *http.Request) {
	render.RenderTemplate(w, "about.page")
}
