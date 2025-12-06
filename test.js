const leftPad = require('left-pad');
const lodash = require('lodash');

console.log("Testing Crabby Logic...");
console.log("Left Pad (10, 0):", leftPad('10', 5, '0'));
console.log("Lodash Version:", lodash.VERSION);
console.log("Success! Crabby provided the ingredients.");
