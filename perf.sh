#!/bin/bash
task="$1"
perfdata="$2"

if [ $task == "flame_graph" ] 
then    
    cargo run flame_graph $perfdata > flame_graph.folded;
    ./flamegraph.pl flame_graph.folded > perf.svg
fi 

if [ $task == "function_counts" ] 
then    
    cargo run function_counts $perfdata > function_counts.txt;
fi 

if [ $task == "function_counts_top_only" ] 
then    
    cargo run function_counts_top_only $perfdata > function_counts.txt;
fi 

if [ $task == "instruction_counts" ] 
then    
    cd perfelf
    for ELF in * ;do 
        if [ $ELF != ".gitignore" ]
        then
            objdump -d $ELF > ../disassembly/$ELF; 
        fi
    done
    cd ..
    cargo run instruction_counts $perfdata > instruction_counts.txt;
fi 



