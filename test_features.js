// Test JavaScript execution
console.log("JavaScript test starting...");

const add = (a, b) => a + b;
console.log("2 + 3 =", add(2, 3));

const items = ["apple", "banana", "cherry"];
console.log("Items:", items.join(", "));

setTimeout(() => {
    console.log("Async works!");
}, 50);

console.log("JavaScript test complete!");
