# greebo
Real time user monitoring service. It pushes data to ElasticSearch for now only compatible client library is keen.io

## Run
Embed on you site keen.io script
```js

<script type="text/javascript" data-cfasync="false">
          !function(name,path,ctx){
              var latest,prev=name!=='Keen'&&window.Keen?window.Keen:false;ctx[name]=ctx[name]||{ready:function(fn){var h=document.getElementsByTagName('head')[0],s=document.createElement('script'),w=window,loaded;s.onload=s.onerror=s.onreadystatechange=function(){if((s.readyState&&!(/^c|loade/.test(s.readyState)))||loaded){return}s.onload=s.onreadystatechange=null;loaded=1;latest=w.Keen;if(prev){w.Keen=prev}else{try{delete w.Keen}catch(e){w.Keen=void 0}}ctx[name]=latest;ctx[name].ready(fn)};s.async=1;s.src=path;h.parentNode.insertBefore(s,h)}}
            }('KeenAsync','https://grebo.host/static/js/stats-1.4.2.min.js',this);

  KeenAsync.ready(function(){
      // Configure a client instance
      var client = new KeenAsync({
            projectId: 'projectId',
            writeKey: 'writeKey'
          });
        client.config.host = "greebo.host";  
        client.initAutoTracking();

    });
    </script>
```

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
