#include <iostream>
#include <fstream>
#include <vector>
#include <string>
using namespace std;

int checkIncreasing(vector<int> x){
    int count = 0;
    for (int i = 1; i < x.size(); ++i) {
        if (x[i-1] < x[i])
            count++;
    }
    return count;
}

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


int main(){
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
    for (auto i: x) 
        cout<<i<<endl;
    //cout<<checkWindow(x)<<endl;
    //cout<<count<<endl;
}