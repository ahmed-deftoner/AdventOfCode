#include <iostream>
#include <fstream>
#include <vector>
#include <string>
using namespace std;

// Day 1a
int checkIncreasing(vector<int> x){
    int count = 0;
    for (int i = 1; i < x.size(); ++i) {
        if (x[i-1] < x[i])
            count++;
    }
    return count;
}

// Day 1b
int checkWindow(vector<int> x) {
    vector<int> sumArr;
    int sum = 0;
    int counter = 0;
    int i = 0;
    while (i <= x.size()) {
        if (counter < 3) {
            sum += x[i];
            counter++;
            i++;
        }else if (counter == 3) {
            sumArr.push_back(sum);
            counter = 0;
            sum = 0;
            i-=2;
        }
    }
    return checkIncreasing(sumArr);
}

// Day 2a
int getPos(vector<int> pos,vector<string> command) {
    int x = 0;
    int y = 0;
    for (int i = 0; i < pos.size(); ++i) {
        if (command[i] == "forward") {
            x += pos[i];
        }else if (command[i] == "up") {
            y -= pos[i];
        }else if (command[i] == "down") {
            y += pos[i];
        }
    }
    return x * y;
}

// Day 2b
int getAim(vector<int> pos,vector<string> command) {
    int depth = 0;
    int aim = 0;
    int x = 0;
    for (int i = 0; i < pos.size(); ++i) {
        if (command[i] == "forward") {
            if (aim != 0)
                depth += aim * pos[i];
            x += pos[i];
        }else if (command[i] == "up") {
            aim -= pos[i];
        }else if (command[i] == "down") {
            aim += pos[i];
        }
    }
    return x * depth;
}

void handle2(){
    fstream fin;
    fin.open("/mnt/e/AdventOfCode/data2.txt", ios::in);
    string s;
    vector<int> x;
    vector<string> command;
    while(!fin.eof()){
        getline(fin, s);
        int pos = s.find(" ");
        command.push_back(s.substr(0, pos));
        x.push_back(stoi(s.substr(pos + 1)));
    }
    cout<<getAim(x,command)<<endl;
}

// Day 3a

void powerConsumption(vector<string> bits) {
    vector<string>::iterator it;
    int len = bits[0].size();
    string gamma;
    string epsilon;
    for (int i = 0; i < len; ++i) {
        int count0 = 0;
        int count1 = 0;
        for (it = bits.begin(); it < bits.end(); it++) {
            string current = *it;
            if (current[i] == '0') 
                count0++;
            else 
                count1++;
        }
        if (count0 > count1) {
            gamma.append("0");
            epsilon.append("1");
        } 
        else {
            gamma.append("1");
            epsilon.append("0");
        }
    }
    cout << gamma << endl;
}

int main(){
    fstream fin;
    fin.open("/mnt/e/AdventOfCode/data3.txt", ios::in);
    string s;
    vector<string> bits;
    while(!fin.eof()){
        getline(fin, s);
        bits.push_back(s);
    }
    powerConsumption(bits);
}