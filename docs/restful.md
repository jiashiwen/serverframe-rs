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

## /api/v1/raw/prefix

* POST

```
{ 
    "prefix": xxx,
    "limit" xx,
}
```

## /api/v1/raw/prefix-revers

* POST

request

```
{ 
    "prefix": xxx,
    "limit" xx,
}
```

response

```
{
	"code": number,
	"msg": string,
	"data": {
	"queryID": xxxx
	"kvs": [{"k":xxx,"v": xxx}]
	}
}

```

## /api/v1/raw/del

* POST

```
{ 
    "Key": "xxx"  
}
```

## /api/v1/txn/opt

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
   {"opt": put
    "keys":["xxx","xxx","xxx"]
   },
   ]
}
```

## /api/v1/txn/get

```
{ 
    "Key": "xxx"  
}
```

## /api/v1/txn/scan

```
{ 
    "begin": "xxx"  ，
    "end"："xxx",
    "limit" xx,
}
```