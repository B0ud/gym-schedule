
## Hot Reload

```
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

## Database
```
docker run --name postgres  -p 5432:5432  -e POSTGRES_DB=gym-schedule -d postgres
```
## Logging

```
MY_APP_LOG=info,gym_schedule=debug systemfd --no-pid -s http::3000 -- cargo watch -x run
```


