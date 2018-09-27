# greebo
Real time user monitoring service. It pushes data to ElasticSearch for now only compatile client library is keen.io

## Run
Configuration

```bash
$ cat config.yml
# configruation for storage
storage:
  url: "http://10.8.0.10:9200"
  type: "elastic"
# index prefx
prefix: "rum"
listen: "127.0.0.1:8081"
clients:
  - project: 'project'
    key: 'key'

```

## Demo
Go to https://mkaciuba.com and check network requests to https://greebo.mkaciuba.com.

## TODO:
* maxmind geo data
* other storages
* handle other clients
* tests!
* refactor
