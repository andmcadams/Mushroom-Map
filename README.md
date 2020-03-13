# Mushroom Map

Check out this project [here](https://andmcadams.github.io/Mushroom-Map/index.html)!

## Usage
The goal of this project is to display sightings of edible (in the culinary sense) mushrooms to aid people looking for places to go forage in the United States and nearby areas. The reason why this is necessary is because current sites that display this data either a) don't allow complex filtering to reduce to edible species or b) don't allow looking over multiple years at a time. For example, iNaturalist (which is where this data is from) allows users to look at a heat map showing hotspots of reported sightings. However, iNaturalist only allows a singular species to be selected at a time or a broad group (all fungi and lichen). This can be somewhat circumvented using a complex list of tags, but that also requires users to know the scientific names of all edible fungi. Fungus names in particular are rather fickle, especially as more DNA based evidence for taxonomic changes is discovered.

In this project, I have already preprocessed the data to remove a majority of fungal species that are not known to be edible. This greatly reduces the number of species that are displayed, allowing users to look through the entire list of species in the sidebar. This does not mean that every species listed in the sidebar is edible, nor does it mean that all edible species are included in this list. I use the term "edible mushrooms" to describe species that are typically characterized as choice edibles in field guides. Therefore, species like the fly agaric (*Amanita muscaria*) and *Gyromitra esculenta* are not listed.

In some cases, I have opted to keep all species of certain genera, as is the case with *Morchella*, due to high rates of edibility throughout the genus and the number of disagreeing species names.

By selecting a month, all of the reports from that month (over all years) will be displayed on the map. The sidebar on the right will show all species that have been reported during that month, allowing users to check/uncheck species that they wish to see or hide.

If you would like to request species be added or removed from this project, please create an issue.

## Creation
This project uses data from iNaturalist to build the map. This data was collected from iNaturalist using their data export tool. Due to size restrictions, I split data up by year. I only selected data from research grade reports with a majority agreements on the identification that were located in the North America region. I also required that the reports had an image associated with them in case I or anyone else wants to verify certain reports. This raw data is located in the data/20\*\*.csv.

This data was then parsed by a Rust program, which can be found in data/src/main.rs. The program goes through the csv files and checks each row to see if the report is of an edible mushroom. Edible mushrooms are defined in this file as an array of strings. If a report has a scientific name that starts with any of the edible mushroom strings, it outputs a row to output.csv containing the date, name, and location (in lat/long coords) of the report. The program checks if the scientific name starts with an edible mushroom name to allow the selection of entire genera, as well as handle edge cases introduced by varities.

This output.csv is read by a javascript function in index.html. When a month is selected, all of the reports associated with that month (regardless of year) will be displayed on the map. This uses the Google Maps JS API V3 to plot the locations and cluster them using the Maps API MarkerClusters.