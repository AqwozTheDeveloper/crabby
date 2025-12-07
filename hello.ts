// Simple TypeScript test
const message: string = "Hello from compiled TypeScript!";
const numbers: number[] = [1, 2, 3, 4, 5];

console.log(message);
console.log("Sum:", numbers.reduce((a, b) => a + b, 0));

interface Person {
    name: string;
    age: number;
}

const person: Person = {
    name: "Crabby",
    age: 1
};

console.log(`${person.name} is ${person.age} year(s) old`);
