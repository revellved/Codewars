function queueTime(customers, n) {
    let tills = Array(n).fill(0);
    for (let time of customers) {
        let nextTill = tills.indexOf(Math.min(...tills));
        tills[nextTill] += time;
    }
    return Math.max(...tills);
}
export {};
//# sourceMappingURL=solution.js.map