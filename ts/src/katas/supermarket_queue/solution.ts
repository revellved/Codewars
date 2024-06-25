
function queueTime(customers: number[], n: number): number {
    let tills: number[] = Array(n).fill(0);

    for (let time of customers) {
        let nextTill = tills.indexOf(Math.min(...tills));
        tills[nextTill] += time;
    }

    return Math.max(...tills);
}
