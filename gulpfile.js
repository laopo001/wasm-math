const { watch, series, task, start } = require('gulp');
const { exec } = require('child_process');

task('watch', function () {
    watch(['src/*.rs'], function (cb) {
        // console.log(123);
        exec('wasm-pack build --dev', function name(err, stdout, stderr) {
            if (err) {
                console.log(err);
            } else if (stdout) {
                console.log(stdout);
            } else if (stderr) {
                console.log(stderr);
            }
            cb();
        })
    });
});

task('default', function () {
    start(['watch'])
});