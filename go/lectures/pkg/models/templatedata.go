package models

// TemplateData holds data for the template
type TemplateData struct {
	StringMap map[string]string
	IntMap    map[string]int
	FloatMap  map[string]float64
	Data      map[string]interface{}
	CSRFToken string
	Error     string
	Warning   string
	Info      string
	Success   string
	Flash     string
}
