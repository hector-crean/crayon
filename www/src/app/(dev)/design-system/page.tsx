"use client";

import React, { useEffect, useState } from 'react';

const ContainerDimensions: React.FC = () => {
  const [dimensions, setDimensions] = useState({ width: '0px', height: '0px' });

  useEffect(() => {
    const cmToPx = (cm: number) => cm * (96 / 2.5399); // More precise conversion

    setDimensions({
      width: `${cmToPx(21)}px`,
      height: `${cmToPx(29.7)}px`,
    });
  }, []);

  const breakpoints = [
    { label: '@xs', size: '20rem', pixels: '320px' },
    { label: '@sm', size: '24rem', pixels: '384px' },
    { label: '@md', size: '28rem', pixels: '448px' },
    { label: '@lg', size: '32rem', pixels: '512px' },
    { label: '@xl', size: '36rem', pixels: '576px' },
    { label: '@2xl', size: '42rem', pixels: '672px' },
    { label: '@3xl', size: '48rem', pixels: '768px' },
    { label: '@4xl', size: '56rem', pixels: '896px' },
    { label: '@5xl', size: '64rem', pixels: '1024px' },
    { label: '@6xl', size: '72rem', pixels: '1152px' },
    { label: '@7xl', size: '80rem', pixels: '1280px' },
  ];

  return (
    <div className="p-4 relative">
      <h1 className="text-2xl font-bold mb-4">Container Dimensions</h1>
      <ul className="space-y-2">
        {breakpoints.map((breakpoint) => (
          <li key={breakpoint.label} className="flex flex-col items-center p-2 border rounded-lg">
            <span className="font-medium mb-2">{breakpoint.label}</span>
            <div
              className="border p-2 relative z-10"
              style={{ 
                width: breakpoint.size, 
                height: '100px', 
                borderStyle: 'dotted', 
                borderColor: '#002FA7', // Yves Klein Blue
                borderWidth: '1px'
              }}
            >
              <span>{breakpoint.size} / {breakpoint.pixels}</span>
              <div
                className="absolute top-0 left-0 w-full h-full bg-white opacity-50 pointer-events-none"
                style={{ 
                  width: dimensions.width, 
                  height: dimensions.height, 
                  borderStyle: 'dotted', 
                  borderColor: '#002FA7' // Yves Klein Blue
                }}
              ></div>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};


const Page = () => {
  return (
    <div>
      <ContainerDimensions />
    </div>
  );
};

export default Page;
