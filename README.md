# Relationship Explorer

Prints out the requested value from a base64 encoded JSON data structure that's
stored in an environmental variable.

## Usage

```
relationship-explorer ENV_VAR_NAME <type>.<name>.var.name
```

## Example

> PLATFORM_RELATIONSHIPS: A base64-encoded JSON object whose keys are the relationship name and the values are arrays of relationship endpoint definitions.

Given the following `PLATFORM_RELATIONSHIPS`:

```
{
    "redis": [
        {
            "service": "rediscache",
            "ip": "1.2.3.4",
            "cluster": "abcdefg123",
            "host": "redis.internal",
            "rel": "redis",
            "scheme": "redis",
            "port": 6379
        }
    ],
    "solr": [
        {
            "service": "solr",
            "ip": "1.2.3.4",
            "cluster": "abcdefg123",
            "host": "solr.internal",
            "rel": "main",
            "path": "solr/mainindex",
            "scheme": "solr",
            "port": 8080
        }
    ],
    "database": [
        {
            "username": "user",
            "scheme": "mysql",
            "service": "mysql",
            "ip": "1.2.3.4",
            "cluster": "abcdefg123",
            "host": "database.internal",
            "rel": "mysql",
            "path": "main",
            "query": {
                "is_master": true
            },
            "password": "",
            "port": 3306
        }
    ]
}

```

You can obtain the **username** from the `database` service named `mysql` using the following:


```
relationship-explorer PLATFORM_RELATIONSHIPS database.mysql.username
```
