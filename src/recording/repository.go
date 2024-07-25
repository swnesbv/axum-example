package recording

import (
    "database/sql"
    "net/http"
    "fmt"
    "time"

)


func qSearch(
    w http.ResponseWriter, conn *sql.DB, start time.Time,end time.Time) (rows *sql.Rows, err error) {

    rows,err = conn.Query(
        "SELECT * FROM schedule WHERE st_hour <= $1 AND en_hour >= $2 AND NOT tsrange($1, $2, '[]') @> ANY(occupied) OR occupied IS NULL", start,end)

    if err != nil {
        switch {
            case true:
            fmt.Fprintf(w, "Error: Query..! : %+v\n", err)
            break
        }
        return
    }
    return rows,err
}


func qRcgAll(
    w http.ResponseWriter, conn *sql.DB) (rows *sql.Rows, err error) {

    rows,err = conn.Query("SELECT * FROM recording")

    if err != nil {
        switch {
            case true:
            fmt.Fprintf(w, "Error: Query..! : %+v\n", err)
            break
        }
        return
    }
    return rows,err
}
