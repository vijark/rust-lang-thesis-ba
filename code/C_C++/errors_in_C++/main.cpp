#include <fstream>
#include <iostream>

using namespace std;

string read_username();

int main() {
    cout << "The username is " << read_username() << "." << endl;

    return 0;
}

string read_username() {
    string name;
    ifstream file;

    file.exceptions(ifstream::failbit | ifstream::badbit);

    try {
        file.open("username.txt");
        getline(file, name);
        file.close();
    } catch (const ifstream::failure &e) {
        cerr << "Exception while reading from file: " << e.what() << endl;
        exit(EXIT_FAILURE);
    }

    return name;
}
