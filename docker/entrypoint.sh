#!/bin/bash
if [[ ! -z "$DEBUG" ]]
then
    echo "Debug requested.  Sleeping infinity."
    /bin/sleep infinity
    exit 1
fi
/herald/herald
