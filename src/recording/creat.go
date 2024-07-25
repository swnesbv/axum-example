package recording

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"html/template"
	"net/http"
	"time"

	// "github.com/lib/pq"

	// "go_authentication/connect"
	// "go_authentication/options"
	"go_authentication/authtoken"
)

func Period(w http.ResponseWriter, r *http.Request) {

	if r.Method == "GET" {

		tpl := template.Must(
			template.ParseFiles(
				"./tpl/navbar.html",
				"./tpl/recording/period.html",
				"./tpl/base.html",
			),
		)
		tpl.ExecuteTemplate(w, "base", nil)
	}

	if r.Method == "POST" {

		loc, _ := time.LoadLocation("UTC")
		start, err := time.ParseInLocation(
			"2006-01-02T15:04:05", r.FormValue("start")+":00", loc)
		end, err := time.ParseInLocation(
			"2006-01-02T15:04:05", r.FormValue("end")+":00", loc)
		if err != nil {
			fmt.Fprintf(w, " Error: time Parse..! : %+v\n", err)
			return
		}

		// if start.After(end) || start.Before(time.Now())  {
		//     fmt.Fprintf(w, " incorrect time interval points..!")
		//     return
		// }

		fmt.Println("start", start)
		fmt.Println("end", end)

		data := authtoken.ListData{
			D_start: start,
			D_end:   end,
		}
		list, err := json.Marshal(data)
		if err != nil {
			fmt.Fprintf(w, " Error: json Marshal..! : %+v\n", err)
			return
		}

		tokenstr := base64.StdEncoding.EncodeToString([]byte(list))

		cookie := http.Cookie{
			Name:     "Period",
			Value:    tokenstr,
			Path:     "/all-sch-search",
			MaxAge:   3600,
			HttpOnly: true,
			Secure:   false,
			SameSite: http.SameSiteLaxMode,
		}
		http.SetCookie(w, &cookie)

		http.Redirect(w, r, "/all-sch-search", http.StatusFound)
		return
	}
}
