#include "constants.h"
#include <math.h>

void setup()
{
  Serial.begin(19200); // set to what need on terminal
  ss.begin(GPSBaud);
  pinMode(pitchPin, OUTPUT);
  pinMode(yawPin, OUTPUT);

  Serial.println("starting up...");
}

void loop()
{
  // Dispatch incoming characters
  while (ss.available() > 0)
    gps.encode(ss.read());

  if (!hotGPS || gps.location.isValid() || gps.altitude.isValid())
  {

    // update dish pos based on offset
    if (hotGPS)
    {
        dishPosition[0] = gps.location.lat() + config.offY;
        dishPosition[1] = gps.location.lng() + config.offX;
        dishPosition[2] = gps.altitude.meters() + config.offset;
    }
    else
    {
        dishPosition[0] = Purdue.staticDishPosition[0];
        dishPosition[1] = Purdue.staticDishPosition[1];
        dishPosition[2] = Purdue.staticDishPosition[2];
    }
    // convert dish data to meters
    dishPosition[0] = config.latConv * dishPosition[0];
    dishPosition[1] = config.longConv * dishPosition[1];

    targetPosition[0] = config.latConv * Purdue.earhart[0];
    targetPosition[1] = config.longConv * Purdue.earhart[1];
    targetPosition[2] = Purdue.earhart[2];

    // combine both vectors
    vectorDir[0] = targetPosition[0] - dishPosition[0];
    vectorDir[1] = targetPosition[1] - dishPosition[1];
    vectorDir[2] = targetPosition[2] - dishPosition[2];

    a = vectorDir[1] > 0; // x direction comparator
    b = vectorDir[0] > 0; // y direction comparator
    angle = atan(vectorDir[0] / vectorDir[1]) * 180. / M_PI;

    // step 1: implement conversion to full circle
    if (a && b) {// Q1
        yaw = angle;
    } else if (!a && b) {// Q2
        yaw = 180. + angle;
    } else if (!a && !b) {// Q3
        yaw = 180. + angle;
    } else if (a && !b) {// Q4
        yaw = 360. + angle;
    }

    yaw = fmod(90. - yaw, 360); // convert to azimuth


    dist = sqrt(vectorDir[0] * vectorDir[0] + vectorDir[1] * vectorDir[1] + vectorDir[2] * vectorDir[2]);
    pitch = asin(vectorDir[2] / dist) * 180 / M_PI;

    // calculations for PWM conversion
    pwmGenPitch = (PWMPitch.pMax - PWMPitch.pMin) * pitch / (PWMPitch.thetaMax - PWMPitch.thetaMin) + PWMPitch.pMin;
    pwmGenYaw = (PWMYaw.pMax - PWMYaw.pMin) * yaw / (PWMYaw.thetaMax - PWMYaw.thetaMin) + PWMYaw.pMin;
    Serial.print("Writing pitch PWM: ");
    Serial.print((pwmGenPitch));
    Serial.print("  Writing yaw PWM: ");
    Serial.println((pwmGenYaw));

    analogWrite(pitchPin, pwmGenPitch);
    analogWrite(yawPin, pwmGenYaw);
  } 
  else
  {
    Serial.println("waiting for GPS signal..."); 
  }
  delay(100);

}