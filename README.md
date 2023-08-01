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
cashcat track CUSTOMER TICKET DURATION
```

tracks a time entry for the given customer, ticket and duration in minutes.
If customer or ticket does not exists. It will be created. They are just represented by a string.

---

```
cashcat list CUSTOMER MONTHOFFSET
```

prints all entries for the given customer for current month - offset.

---

```
cashcat export CUSTOMER MONTHOFFSET DESTINATION
```

exports all entries for the given customer for current month - offset to the
given destination as json.

---

```
cashcat delete ID
```

deletes the given time entry by ID. Use the List Command to see all entries with ID.

---

```
cashcat backup DESTINATION
```

copies the sqlite databse to the given destination
