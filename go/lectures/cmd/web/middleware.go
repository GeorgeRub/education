package main

import (
	"fmt"
	"github.com/justinas/nosurf"
	"net/http"
)

// WriteToConsole logs information about the request to the console.
//
// It logs the request method and URL, as well as the Accept, Accept-Encoding,
// and User-Agent headers.
func WriteToConsole(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("Hit the page")
		fmt.Println(r.Header.Get("Accept"))
		fmt.Println(r.Header.Get("Accept-Encoding"))
		fmt.Println(r.Header.Get("User-Agent"))
		next.ServeHTTP(w, r)
	})
}

// NoSurf is the csrf protection middleware
func NoSurf(next http.Handler) http.Handler {
	csrfHandler := nosurf.New(next)

	csrfHandler.SetBaseCookie(http.Cookie{
		HttpOnly: true,
		Path:     "/",
		Secure:   app.InProduction,
		SameSite: http.SameSiteLaxMode,
	})
	return csrfHandler
}

func SessionLoad(next http.Handler) http.Handler {
	return session.LoadAndSave(next)
}
