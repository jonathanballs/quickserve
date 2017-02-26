var fs = require('fs');
var crypto = require('crypto');
var inspect = require('util').inspect;

var ssh2 = require('ssh2');
var utils = ssh2.utils;

new ssh2.Server({
  hostKeys: []
}, function(client) {
  console.log('Client connected!');

  client.on('authentication', function(ctx) {
    if (ctx.method === 'password'
        && ctx.username === 'foo'
        && ctx.password === 'bar')
      ctx.accept();
  }).on('ready', function() {
    console.log('Client authenticated!');

    client.on('session', function(accept, reject) {
      var session = accept();
      session.once('exec', function(accept, reject, info) {
        console.log('Client wants to execute: ' + inspect(info.command));
        var stream = accept();
        stream.stderr.write('Oh no, the dreaded errors!\n');
        stream.write('Just kidding about the errors!\n');
        stream.exit(0);
        stream.end();
      });
    });
  }).on('end', function() {
    console.log('Client disconnected');
  });
}).listen(0, '127.0.0.1', function() {
  console.log('Listening on port ' + this.address().port);
});
