#include <TinyGPSPlus.h>
#include <SoftwareSerial.h>


int hotGPS = 1; // use GPS? 1 yes, 0 use test cases

// typedef ___what you want to change the name of___ __what you want to change it to__;

// GPS control pins
static const int RXPin = 4, TXPin = 3; //RXPin is TX on gps
static const uint32_t GPSBaud = 9600;

// motor control pins
int pitchPin = 9;
int yawPin = 10;

double signalYaw;
double signalPitch;

double vectorDir[3];
double dishPosition[3];
double targetPosition[3];

double angle;
double yaw;
double dist;
double pitch;
double pwmGenPitch;
double pwmGenYaw;
int a;
int b;

// The TinyGPSPlus object
TinyGPSPlus gps;

// The serial connection to the GPS device
SoftwareSerial ss(RXPin, TXPin);

// dish config
typedef struct dish {
  double offset = 1; // dish motor offset from GPS in z dir
  double offX = 0; // dish GPS offset from motor in west/east
  double offY = 0; // dish GPS offset from motor in north/south
  double latConv = 110066.3829; // conversion ratio from lat deg to meters
  double longConv = 90369.2356; // conversion ratio from long deg to meters
} dish;

// servo constants for yaw
struct PWM {
  double pMax; // PWM max value
  double pMin; // PWM min value
  double thetaMax; // Max angle
  double thetaMin; // Min angle
};

// test cases for targets, read comments and select one
struct targets {
  double arms[3] = {40.431156, -86.915763, 201.40};  // position of Neil Armstrong's Crotch
  double pmu[3] = {40.424083, -86.910614, 183.70}; // position of middle of PMU intersection
  double bell[3] = {40.427314, -86.913871, 218.10}; // position of under bell tower
  double earhart[3] = {40.425521, -86.925102, 193.80}; // position of Amelia Earhart's Feet
  double staticDishPosition[3] = {40.427364, -86.919029, 184.60}; // position of dish at bechtel
};

dish config;
struct PWM PWMPitch = {250, 160, 90, 0};
struct PWM PWMYaw = {255, 0, 360, 0};
struct targets Purdue;

