# Cash Cat

CLI tool for tracking time and exporting summaries as json.

Data is stored in a sqlite databse inside your home directory.

This project is work in progress and may change rapidly.

# Commands

```
cashcat init
```

Initialize <b>or reset</b> your cash cat database.

---

```
cashcat track -c CUSTOMER -t TICKET -d DURATION
```

tracks a time entry for the given customer, ticket and duration in minutes.
If customer or ticket does not exists. It will be created.

---

```
cashcat list -c CUSTOMER -m MONTHOFFSET
```

prints all entries for the given customer for current month - offset.

---

```
cashcat export -c CUSTOMER -m MONTHOFFSET -d DESTINATION
```

exports all entries for the given customer for current month - offset to the
given destination as json.

---

```
cashcat delete -i ID
```

deletes the given time entry by ID. Use the List Command to see all entries with ID.

---

```
cashcat backup -d DESTINATION
```

copies the sqlite databse to the given destination
