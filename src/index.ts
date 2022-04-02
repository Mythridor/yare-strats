/**
 * This file is the entry point for your bot.
 */

import RenderService from 'yare-code-sync/client/RenderService'

RenderService.circle(my_spirits[0], 100);
// Your code goes here
// Activate it by pressing the button below or with SHIFT+ENTER

export const Distance = (p1: Position, p2: Position): number => {
    return Math.sqrt(
        Math.pow(p2[0] - p1[0], 2) +
        Math.pow(p2[1] - p1[1], 2)
    )
}

function findClosestStar(): LargeStar {
    return Distance(
        base.position,
        star_a1c.position
    ) < Distance(
        base.position,
        star_zxq.position
    ) ? star_a1c : star_zxq
}

export const ClosestStar = findClosestStar()

var size = my_spirits.length / 3

var worker_fleet_A = my_spirits.slice(0, size)
var worker_fleet_B = my_spirits.slice(size, 2*size)

var worker_fleet = [worker_fleet_A, worker_fleet_B]

var soldier_fleet = my_spirits.slice(2*size, my_spirits.length)

for (var fleet of worker_fleet) {
    for (var worker of fleet) {
        var target;
    
        if(worker_fleet_A.includes(worker)) {
            target = ClosestStar
        } else if (worker_fleet_B.includes(worker)) {
            target = ClosestStar
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
    var soldier_target = base;
    var soldier_closest_star = ClosestStar;

    var enemy_in_sight = soldier.sight.enemies;
    var closest_ennemy = soldier.sight.enemies[0]
    var structure_in_sight = soldier.sight.structures;
    
    console.log("Stats:")
    console.log("--------------------------------------")
    console.log("Workers:", worker_fleet_A.length + worker_fleet_B.length -2)
    console.log("Fleet A:", worker_fleet_A.length-1)
    console.log("Fleet B:", worker_fleet_B.length-1)
    console.log("Soldiers:")
    console.log("Fleet A:", soldier_fleet.length-1)
    console.log("Enemies in sight:", enemy_in_sight)
    console.log("Closest enemy:", enemy_in_sight[0])

    switch (soldier.mark) {
        case 'harvester':
            soldier.move(soldier_closest_star.position);
            soldier.energize(soldier);
        case 'attacker':
            soldier.move(soldier_target.position)

            if (closest_ennemy || soldier.energy > 5) {
                soldier.energize(closest_ennemy);
            } else {
                soldier.set_mark("harvester")
            }
    }

    if (soldier.energy <= soldier.energy_capacity) {
            soldier.set_mark("harvester")
    }
    
    if (enemy_in_sight.length > 0){
        if (soldier.energy <= soldier.energy_capacity) {
            soldier.move(soldier_closest_star.position)
            soldier.set_mark("harvester")
        }
        if (enemy_in_sight && soldier.energy > 5) {
            soldier.set_mark("attacker")
        }
    }
    
    if (structure_in_sight.length > 0){
        var invader = structure_in_sight[0]
        
        if (soldier.energy >= 5) soldier.set_mark("attacker")
        if (soldier.mark == "attacker") {
            if (structure_in_sight.length > 0) {
                soldier.move(structure_in_sight[0])
                soldier.energize(invader)
            }
        }
    }
}