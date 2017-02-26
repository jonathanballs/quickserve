var fs = require('fs');
var crypto = require('crypto');
var inspect = require('util').inspect;

var ssh2 = require('ssh2');
var utils = ssh2.utils;

function main()
{
    new ssh2.Server({
      hostKeys: [fs.readFileSync('host.key')]
    }, function(client) {
      console.log('Client connected!');

      client.on('authentication', function(ctx) {
          ctx.accept();
      }).on('ready', function() {

        client.on('session', function(accept, reject) {
          var session = accept();
          session.write("hello");
          //session.once('exec', function(accept, reject, info) {
            //console.log('Client wants to execute: ' + inspect(info.command));
            //var stream = accept();
            //stream.stderr.write('Oh no, the dreaded errors!\n');
            //stream.write('Just kidding about the errors!\n');
            //stream.exit(0);
            //stream.end();
          //});

          session.once('forwardOut', function(accept, reject, info) {
            console.log("Creating port forwarding");
            var stream = accept();

            return true;
          });
        });
      }).on('end', function() {
        console.log('Client disconnected');
      });
    }).listen(2222, '127.0.0.1', function() {
      console.log('Listening on port ' + this.address().port);
    });
};
main()

module.exports = main
