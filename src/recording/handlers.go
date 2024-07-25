package recording

import (
    //"fmt"
    //"time"
    "net/http"
    "html/template"

    //"github.com/lib/pq"

    //"go_authentication/options"
    "go_authentication/connect"
    "go_authentication/authtoken"
)


func Search(w http.ResponseWriter, r *http.Request) {

    if r.Method == "GET" {

        i,err := authtoken.TokenSearch(w,r)
        if i == nil {
            return
        }
        if err != nil {
            return
        }

        conn := connect.ConnSql()
        rows,err := qSearch(w, conn,i.D_start,i.D_end)
        if err != nil {
            return
        }
        list,err := rcgSearch(w, rows)
        if err != nil {
            return
        }
        defer conn.Close()

        tpl := template.Must(template.ParseFiles("./tpl/navbar.html", "./tpl/recording/search.html", "./tpl/base.html" ))

        tpl.ExecuteTemplate(w, "base", list)
    }
}


func RcgAll(w http.ResponseWriter, r *http.Request) {

    if r.Method == "GET" {

        conn := connect.ConnSql()
        rows,err := qRcgAll(w, conn)
        if err != nil {
            return
        }
        list,err := allRcg(w, rows)
        if err != nil {
            return
        }

        defer conn.Close()

        tpl := template.Must(template.ParseFiles("./tpl/navbar.html", "./tpl/recording/all.html", "./tpl/base.html" ))

        tpl.ExecuteTemplate(w, "base", list)
    }
}

