package render

import (
	"bytes"
	"github.com/george/web/pkg/config"
	"github.com/george/web/pkg/models"
	"html/template"
	"log"
	"net/http"
	"path/filepath"
)

var app *config.AppConfig

// NewTemplates initializes the package-level AppConfig pointer.
//
// It sets the global `app` variable to the provided `AppConfig` instance,
// allowing the render package to access application-wide configuration settings.
func NewTemplates(a *config.AppConfig) {
	app = a
}

// AddDefaultData adds default data to a models.TemplateData instance.
//
// It takes a pointer to a models.TemplateData as an argument and adds default
// data to it. The default data consists of a map of strings, where the keys are
// "about" and "home", and the values are "Hello, again for about." and
// "Hello, again for home.", respectively. The function returns the updated
// models.TemplateData pointer.
func AddDefaultData(td *models.TemplateData) *models.TemplateData {
	stringMap := make(map[string]string)
	stringMap["about"] = "Hello, again for about."
	stringMap["home"] = "Hello, again for home."
	//td.StringMap = stringMap
	return td
}

// RenderTemplate renders the specified HTML template and writes it to the ResponseWriter.
//
// It takes a http.ResponseWriter and the name of the template file (without extension)
// as arguments. The function relies on a template cache to avoid reparsing templates
// on each request. If the specified template cannot be found in the cache, or if
// there is an error during template execution, the function will log the error and
// terminate the application.
func RenderTemplate(w http.ResponseWriter, tmpl string, td *models.TemplateData) {
	var tc map[string]*template.Template
	if app.UseCache {
		// get the template cache from the app config
		tc = app.TemplateCache
	} else {
		tc, _ = CreateTemplateCache()
	}

	// get the template cache from the app config

	// get the template from the cache
	t, ok := tc[tmpl]
	if !ok {
		log.Fatal("Could not get template from template cache")
	}

	buf := new(bytes.Buffer)

	td = AddDefaultData(td)

	err := t.Execute(buf, td)
	if err != nil {
		log.Fatalln(err)
	}

	//render the template

	_, err = buf.WriteTo(w)
	if err != nil {
		log.Fatalln(err)
	}

}

// CreateTemplateCache creates a map of template names to *template.Template
// objects. It searches for all files in the ./templates directory ending with
// *.page.html and creates a template for each one. If a file is found with the
// same name as the page but with a .layout.html suffix, it adds that template
// to the cache as well.
//
// The cache is returned as a map[string]*template.Template, where the key is
// the name of the template and the value is a pointer to the *template.Template
// object.
func CreateTemplateCache() (map[string]*template.Template, error) {
	myCache := map[string]*template.Template{}

	// get all the files named *.page.html from ./templates
	pages, err := filepath.Glob("./templates/*.page.html")
	if err != nil {
		return myCache, err
	}

	// range through all files ending with *.page.html
	for _, page := range pages {
		name := filepath.Base(page)
		ts, err := template.New(name).ParseFiles(page)
		if err != nil {
			return myCache, err
		}
		matches, err := filepath.Glob("./templates/*.layout.html")
		if err != nil {
			return myCache, err
		}

		if len(matches) > 0 {
			ts, err = ts.ParseGlob("./templates/*.layout.html")
			if err != nil {
				return myCache, err
			}
		}

		myCache[name] = ts
	}
	return myCache, nil
}
