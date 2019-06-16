#include <iostream>

using namespace std;

class Rectangle {
  private:
    double length, width;

  public:
    string name;
    Rectangle(double, double);
    void set_length(float);
    void set_width(float);
    double get_length();
    double get_width();
    float perimeter();
    float area();
    friend ostream &operator<<(ostream &, const Rectangle &);
    bool operator==(Rectangle);
    bool operator!=(Rectangle);
};

Rectangle::Rectangle(double len, double wid) {
    name = "";
    length = len;
    width = wid;
}

void Rectangle::set_length(float len) { length = len; }

void Rectangle::set_width(float wid) { width = wid; }

double Rectangle::get_width() { return width; }

double Rectangle::get_length() { return length; }

float Rectangle::perimeter() { return (2 * length + 2 * width); }

float Rectangle::area() { return length * width; }

ostream &operator<<(ostream &out, const Rectangle &data) {
    out << data.name << "{Length: " << data.length << ", Width: " << data.width
        << "}";
    return out;
}

bool Rectangle::operator==(Rectangle other) {
    return name == other.name && length == other.length && width == other.width;
}

bool Rectangle::operator!=(Rectangle other) { return !(*this == other); }

int main() {
    // create objects with constructor
    Rectangle first(5, 2.5);
    Rectangle second(8, 18.9);

    // access and modify data marked as public
    first.name = "Alice";
    second.name = "Bob";

    // print objects
    cout << "First rectangle: " << first;
    cout << endl
         << "Area: " << first.area() << ", Perimeter: " << first.perimeter()
         << endl
         << endl;

    cout << "Second rectangle: " << second;
    cout << endl
         << "Area: " << second.area() << ", Perimeter: " << second.perimeter()
         << endl
         << endl;

    // compare objects
    if (first == second) {
        cout << "Rectangles are equal" << endl;
    } else {
        cout << "Rectangles are not equal" << endl;
    }

    cout << endl << "changing values..." << endl;

    first.name = second.name;
    first.set_length(8);
    second.set_width(2.5);

    if (first == second) {
        cout << "Rectangles are equal" << endl;
    } else {
        cout << "Rectangles are not equal" << endl;
    }

    return 0;
}
