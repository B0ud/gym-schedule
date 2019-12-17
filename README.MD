
## Hot Reload

```
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

docker run --name postgres  -p 5432:5432 -d postgres