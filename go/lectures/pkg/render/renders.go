package render

import (
	"html/template"
	"log"
	"net/http"
)

// RenderTemplate renders a template with the given name and writes the result
// to the given ResponseWriter.
//
// It first parses the named template and the base layout template. Then it
// executes the parsed template and writes the result to the given
// ResponseWriter. If any error occurs during the execution of the parsed
// template, it logs an error message and returns.
func RenderTemplate(w http.ResponseWriter, tmpl string) {
	parsedTemplate, _ := template.ParseFiles("./templates/"+tmpl+".html", "./templates/base.layout.html")
	err := parsedTemplate.Execute(w, nil)
	if err != nil {
		log.Fatalln(err)
		return
	}

}
