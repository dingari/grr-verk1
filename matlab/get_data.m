function [k, t, sorted] = get_data(filename)

    delimiter = ',';
    startRow = 3;

    formatSpec = '%f%f%[^\n\r]';
    fileID = fopen(filename,'r');

    dataArray = textscan(fileID, formatSpec, 'Delimiter', delimiter, 'HeaderLines' ,startRow-1, 'ReturnOnError', false);
    
    fclose(fileID);

    k = dataArray{:, 1};
    t = dataArray{:, 2};

    clearvars filename delimiter startRow formatSpec fileID dataArray ans;