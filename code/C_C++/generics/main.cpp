#include <iostream>
#include <string>

using namespace std;

template <class T>
class Point {
  public:
    T x, y;
    Point(T x, T y);
};

template <class T>
Point<T>::Point(T x_input, T y_input) {
    x = x_input;
    y = y_input;
}

class Animal {
  public:
    virtual string noise() = 0;
};

class Sheep : public Animal {
  public:
    string noise() {
        return "baaaaaah";
    }
};

template <class T>
void animal_noise(T &animal) {
    cout << "This animal makes " << animal.noise() << "." << endl;
}

class temp {};

int main() {
    Point<int> p1 = Point<int>(1, 2);
    Point<double> p2 = Point<double>(1.0, 2.0);
    Point<char> p3 = Point<char>('x', 'y');

    cout << "Point { x: " << p1.x << ", y: " << p1.y << " }" << endl;
    cout << "Point { x: " << p2.x << ", y: " << p2.y << " }" << endl;
    cout << "Point { x: " << p3.x << ", y: " << p3.y << " }" << endl;

    Sheep animal = Sheep();
    animal_noise(animal);

    return 0;
}
