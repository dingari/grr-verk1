clear all; close all; clc;

%% 1 - Random - 100.000 elements
[k1, t1] = get_data('random_1.csv');
[k2, t2] = get_data('random_2.csv');
[k3, t3] = get_data('random_3.csv');
[k4, t4] = get_data('random_4.csv');
[k5, t5] = get_data('random_5.csv');

figure(1);
plot(k1, t1, 'bo'); hold on;
plot(k2, t2, 'ro'); hold on;
plot(k3, t3, 'ko'); hold on;
plot(k4, t4, 'mo'); hold on;
plot(k5, t5, 'go'); hold on;
title('Randomly distributed array');
xlabel('k');
ylabel('time [ms]');

t_avg = (t1 + t2 + t3 + t4 + t5)/5;

figure(2);
plot(k1, t_avg);
title('Randomly distributed array - Average');
xlabel('k');
ylabel('time [ms]');

%% 2 - Equal distribution -
clear all;
[k1, t1] = get_data('equal_1.csv');
[k2, t2] = get_data('equal_2.csv');
[k3, t3] = get_data('equal_3.csv');
[k4, t4] = get_data('equal_4.csv');
[k5, t5] = get_data('equal_5.csv');

figure(3);
plot(k1, t1, 'bo'); hold on;
plot(k2, t2, 'ro'); hold on;
plot(k3, t3, 'ko'); hold on;
plot(k4, t4, 'mo'); hold on;
plot(k5, t5, 'go'); hold on;
title('Equally distributed array');
xlabel('k');
ylabel('time [ms]');

t_avg = (t1 + t2 + t3 + t4 + t5)/5;

figure(4);
plot(k1, t_avg);
title('Equally distributed array - Average');
xlabel('k');
ylabel('time [ms]');


%% 3 - Almost sorted - 7.000 elements
clear all;
[k1, t1] = get_data('almost_1.csv');
[k2, t2] = get_data('almost_2.csv');
[k3, t3] = get_data('almost_3.csv');
[k4, t4] = get_data('almost_4.csv');
[k5, t5] = get_data('almost_5.csv');

figure(5);
plot(k1, t1, 'bo'); hold on;
plot(k2, t2, 'ro'); hold on;
plot(k3, t3, 'ko'); hold on;
plot(k4, t4, 'mo'); hold on;
plot(k5, t5, 'go'); hold on;
title('Almost sorted array');
xlabel('k');
ylabel('time [ms]');

t_avg = (t1 + t2 + t3 + t4 + t5)/5;

figure(6);
plot(k1, t_avg);
title('Almost sorted array - Average');
xlabel('k');
ylabel('time [ms]');
