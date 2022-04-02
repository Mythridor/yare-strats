// Your code goes here
// Activate it by pressing the button below or with SHIFT+ENTER

size = my_spirits.length / 3

var worker_fleet_A = my_spirits.slice(0, size)
var worker_fleet_B = my_spirits.slice(size, 2*size)

var worker_fleet = [worker_fleet_A, worker_fleet_B]

var soldier_fleet = my_spirits.slice(2*size, my_spirits.length)
var enemy_in_sight;

for (var fleet of worker_fleet) {
    for (var worker of fleet) {
        var target;
    
        if(worker_fleet_A.includes(worker)) {
            target = star_p89
        } else if (worker_fleet_B.includes(worker)) {
            target = star_zxq
        } else {
            target = base
        }
    
    
        if (worker.energy == worker.energy_capacity) {
            worker.move(base.position)
            worker.energize(base)
        }
    
        if (worker.energy == worker.energy_capacity){
            worker.set_mark("charging")
        } else if (worker.energy == 0){
            worker.set_mark("harvesting")
        }
    
        if (worker.mark == "charging"){
            worker.move(base.position)
            worker.energize(base)
        } else {
            worker.move(target.position)
            worker.energize(worker)
        }
    }
}


for (var soldier of soldier_fleet) {
    enemy_in_sight = soldier.sight.enemies
    soldier.move(star_zxq.position)
    if (soldier.sight.enemies.length > 0){
        var invader = soldier.sight.enemies[0]
        if (soldier.energy > 25) soldier.set_mark("attacker")
        if (soldier.mark == "attacker") {
            soldier.move(invader.position)
            soldier.energize(invader)
        }
    }
}



console.log("Stats:")
console.log("--------------------------------------")
console.log("workers:")
console.log("Fleet A:", worker_fleet_A.length-1)
console.log("Fleet B:", worker_fleet_B.length-1)
console.log("Soldiers:")
console.log("Fleet A:", soldier_fleet.length-1)
console.log("Enemies in sight:", enemy_in_sight)