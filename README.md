# tzf in Cloudflare Workers

```bash
npx wrangler dev
```

TODO: Fix error:

```
the trait bound `fn(Query<Params>) -> impl Future<Output = impl axum::response::IntoResponse> {get_tz}: Handler<_, _>` is not satisfied
the following other types implement trait `Handler<T, S>`:
  <Layered<L, H, T, S> as Handler<T, S>>
  <MethodRouter<S> as Handler<(), S>>
```
