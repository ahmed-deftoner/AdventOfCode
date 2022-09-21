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

int main(){
    fstream fin;
    fin.open("data.txt", ios::in);
    string s;
    vector<int> x;
    while(!fin.eof()){
        getline(fin, s);
        x.push_back(stoi(s));
    }
    int count = checkIncreasing(x);
    cout<<count<<endl;
}