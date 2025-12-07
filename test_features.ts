// Test TypeScript features
interface Person {
    name: string;
    age: number;
}

const greet = (person: Person): string => {
    return `Hello, ${person.name}! You are ${person.age} years old.`;
};

const user: Person = {
    name: "Crabby User",
    age: 25
};

console.log(greet(user));

// Test array methods
const numbers: number[] = [1, 2, 3, 4, 5];
const doubled = numbers.map(n => n * 2);
console.log("Doubled:", doubled);

// Test async/await
async function fetchData(): Promise<string> {
    return new Promise((resolve) => {
        setTimeout(() => resolve("Data loaded!"), 100);
    });
}

fetchData().then(data => console.log(data));
