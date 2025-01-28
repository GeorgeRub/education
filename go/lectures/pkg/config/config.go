package config

import (
	"github.com/alexedwards/scs/v2"
	"html/template"
)

type AppConfig struct {
	UseCache      bool
	TemplateCache map[string]*template.Template
	Port          string
	InProduction  bool
	Session       *scs.SessionManager
	//InfoLog       *log.Logger
	//ErrorLog      *log.Logger
}
