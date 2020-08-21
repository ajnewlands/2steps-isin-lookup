# 2steps-isin-lookup
This is intended to be a simple example of constructing a custom 2 Steps command.

This demonstrates the use of metadata to define the input and output contract between 2 Steps and the custom command, i.e. 

```
{
  "package": "ISIN lookup",  
  "major": 1,
  "minor": 2,
  "arch": "AMD64",
  "run": "bin/isin_lookup",
  "os": "Linux",
  "description": "Look up vendor codes by ISIN",
  "input": [
    { 
      "name": "isin",
      "type": "string",
      "optional": false,
      "description": "ISIN number to look up"
    }
  ],
  "output": [
    { 
      "name": "ric_code", 
      "description": "Retrieved RIC code" 
    },
    { 
      "name": "bbg_code",
      "description": "Retrieved BBG code" 
    }
  ]
}
```

This defines an external command called "ISIN lookup, version 1.2, which requires one input parameter "isin" and will either generate two output parameters, "ric_code" and "bbg_code" or an error.

As such, for a properly formed and valid input such as 
```
{ 
  "isin": "AU000000CBA7"
}
```

The custom command should generate output of the form 
```
{
  "results": { 
    "ric_code":"CBA.AX",
    "bbg_code":"CBA:AU"
  }
}
```

If instead there is a problem, the custom command should generate input of the following form (in addition to setting an appropriate exit code).

```
{
  "results": {},
  "error ": "Couldn't find a ticker corresponding to ISIN 123456789ABC"
}
```

This example can be compiled and placed in an appropriate directory structure along with the supporting data file, like so
```
isin_lookup/bin/isin_lookup
isin_lookup/data/isins.tsv
```

This can be converted into a signed 2 Steps package and uploaded using the script packaging utility to make it available within 2 Steps.
```
2steps-script-pack 
  --manifest ./manifest.json 
  --output ./isin_lookup.2st 
  --directory ./isin_lookup 
  --private-key ./private.pem 
  --public-key ./public.pem 
  --upload
```
