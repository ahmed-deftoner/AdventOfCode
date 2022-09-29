#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <math.h>
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

int binaryToDecimal(string binary) {
    int num = 0;
    int i = 0;
    for (int it = binary.length() - 1; it >= 0; it--) {
        if (binary[it] == '1') {
            num += pow(2.0, i);
        }
        i++;
    }
    return num;
}

// Day 3a
int powerConsumption(vector<string> bits) {
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
    return binaryToDecimal(gamma) * binaryToDecimal(epsilon); 
}

// Day 3b
string carbomRating(vector<string> &bits) {
    vector<string> carbon(bits);
    vector<string>::iterator it;
    int len = bits[0].size();
    for (int i = 0; i < len; ++i) {
        int count0 = 0;
        int count1 = 0;
        vector<int> index;
        for (it = carbon.begin(); it < carbon.end(); it++) {
            string current = *it;
            if (current[i] == '0') 
                count0++;
            else 
                count1++;
        }
        if (count0 < count1) {
            for (int j = 0; j < carbon.size(); j++) {
                if (carbon[j][i] == '0')
                    index.push_back(j);
            }
           vector<string> x;
            for (int j = 0; j < index.size(); j++) 
                x.push_back(carbon[index[j]]);
            carbon = x;
        } else if (count0 > count1) {
            for (int j = 0; j < carbon.size(); j++) {
                if (carbon[j][i] == '1')
                    index.push_back(j);
            }
            vector<string> x;
            for (int j = 0; j < index.size(); j++) 
                x.push_back(carbon[index[j]]);
            carbon = x;
        }else {
            for (int j = 0; j < carbon.size(); j++) {
                if (carbon[j][i] == '0')
                    index.push_back(j);
            }
            vector<string> x;
            for (int j = 0; j < index.size(); j++) {
                x.push_back(carbon[index[j]]);
            }
            carbon = x;
        }
    }
    return carbon[0];
}

// Day 3b
string oxygenRating(vector<string> &bits) {
    vector<string> oxygen(bits);
    vector<string> carbon(bits);
    vector<string>::iterator it;
    int len = bits[0].size();
    for (int i = 0; i < len; ++i) {
        int count0 = 0;
        int count1 = 0;
        vector<int> index;
        for (it = oxygen.begin(); it < oxygen.end(); it++) {
            string current = *it;
            if (current[i] == '0') 
                count0++;
            else 
                count1++;
        }
        if (count0 > count1) {
            for (int j = 0; j < oxygen.size(); j++) {
                if (oxygen[j][i] == '0')
                    index.push_back(j);
            }
           vector<string> x;
            for (int j = 0; j < index.size(); j++) 
                x.push_back(oxygen[index[j]]);
            oxygen = x;
        } else if (count0 < count1) {
            for (int j = 0; j < oxygen.size(); j++) {
                if (oxygen[j][i] == '1')
                    index.push_back(j);
            }
            vector<string> x;
            for (int j = 0; j < index.size(); j++) 
                x.push_back(oxygen[index[j]]);
            oxygen = x;
        }else {
            for (int j = 0; j < oxygen.size(); j++) {
                if (oxygen[j][i] == '1')
                    index.push_back(j);
            }
            vector<string> x;
            for (int j = 0; j < index.size(); j++) {
                x.push_back(oxygen[index[j]]);
            }
            oxygen = x;
        }
    }
    return oxygen[0];
}

// Day 3b
int lifeSupportRating(vector<string> &bits) {
    return binaryToDecimal(oxygenRating(bits)) * binaryToDecimal(carbomRating(bits)); 
}

void handle3() {
    fstream fin;
    fin.open("/mnt/e/AdventOfCode/data3.txt", ios::in);
    string s;
    vector<string> bits;
    while(!fin.eof()){
        getline(fin, s);
        bits.push_back(s);
    }
    ///cout << powerConsumption(bits) << endl;
    cout<<lifeSupportRating(bits);
}

// Day 4a
struct Board
{
    int arr[5][5];
};

bool checkBingo(Board b) {
    // check vertical
    for (int i = 0; i < 5; i++) {
        bool bingoy = true;
        for (int j = 0; j < 5; j++) {
           if (b.arr[i][j] != -1) {
                bingoy = false;
                break;
           } 
        }
        if (bingoy == true) 
            return true;
    }
    // check horizontal
    for (int i = 0; i < 5; i++) {
        bool bingox = true;
        for (int j = 0; j < 5; j++) {
           if (b.arr[j][i] != -1) {
                bingox = false;
                break;
           } 
        }
        if (bingox == true) 
            return true;
    }
    // check diagonal
    bool bingod = true;
    for (int i = 0; i < 5; i++) {
        for (int j = 0; j < 5 && i == j; j++) {
           if (b.arr[i][j] != -1) {
                bingod = false;
                break;
           } 
        }
    }
    if (bingod == true) 
       return true;
    // check reverse diagonal
    bool bingor = true;
    for (int i = 0; i < 5; i++) {
        for (int j = 0; j < 5 && i + j - 5; j++) {
           if (b.arr[i][j] != -1) {
                bingor = false;
                break;
           } 
        }
    }
    if (bingor == true) 
        return true;
    return false;
}

int unmarkedSum


void finalScore(vector<Board> &b, vector<int> moves) {
    vector<Board>::iterator it;
    for (int i = 0; i < moves.size(); i++) {
        for (it = b.begin(); it != b.end(); ++it) {
            int count = 0;
            for (int a = 0; a < 5; ++a) {
                for (int b = 0; b < 5; ++b) {
                    if (it->arr[a][b] == moves[i]) {
                        it->arr[a][b] = -1;
                        count++;
                    }
                }
            }
            bool a = checkBingo(*it);
            if (a == true) 
                cout << moves[i]<< endl;
        }
    }
    
}

int main(){
    fstream fin;
    fin.open("/mnt/e/AdventOfCode/data4.txt", ios::in);
    string s;
    vector<int> moves;
    vector<Board> boards;
    getline(fin, s);
    int x = 0;
    for (int i = 0; i <= s.length(); i++) {
        if (s[i] == ',') {
            moves.push_back(stoi(s.substr(x, i)));
            x = i + 1;
        }
    }
    moves.push_back(stoi(s.substr(x)));
    while(!fin.eof()) {
        Board b;
        for (int i = 0; i < 5; ++i)
            for (int j = 0; j < 5; ++j)
                fin >> b.arr[i][j];
        boards.push_back(b);
    }
    finalScore(boards, moves);
}