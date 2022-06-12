# todo-cli
🧾To do app using 🦀Rustlang 

- [x] ~~Make todo app with Rust🦀 v-0.2~~
- [x] ~~Build an application~~
- [x] ~~Improve visualization adding color~~
- [x] ~~Implement a run a loop and ask the user for their comman every iteration~~
- [x] ~~Implement a comman for changing the task description~~
- [ ] Implement a custom sort command (Priority, Due date ...)
- [ ] Implement a cPush task data to a todo.data

## Database
**Start the database**
```sh
docker run --name pg-dev -e POSTGRES_PASSWORD=340834 -p 5432:5432 postgres:alpine
```
**optional psql (other terminal)**
```sh
docker exec -it postgres pg-dev psql
```
