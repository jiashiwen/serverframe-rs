# restful 接口文档

## /api/v1/raw/put

* POST

```
{ 
    "Key": "xxx"
    "Value": "xxx"
  
}
```

## /api/v1/raw/get

* POST

```
{ 
    "Key": "xxx"  
}
```

## /api/v1/raw/scan

* POST

```
{ 
    "begin": "xxx"  ，
    "end"："xxx",
    "limit" xx,
}
```

## /api/v1/raw/del

* POST

```
{ 
    "Key": "xxx"  
}
```

## /api/v1/txn

* POST

```
{ 
   [
   {"opt": put
    "kvs":[{"k": "xx","v":"xx"},{"k": "xx","v":"xx"},{"k": "xx","v":"xx"}]
   },
   {"opt": del
    "kvs":[{"k": "xx","v":"xx"},{"k": "xx","v":"xx"},{"k": "xx","v":"xx"}]
   },
   {"opt": get
    "keys":["xxx","xxx","xxx"]
   },
   ]
}
```