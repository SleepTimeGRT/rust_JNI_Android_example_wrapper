package com.example.android;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    //JNI functions

    //func 1: returns i32 x 2
    private static native int returni32(final int i32);

    //func 2: returns false
    private static native boolean getFileStatus();

    //func 3: concatenates the "string" with another String and returns the result
    private static native String returnJString(final String string);

    //func 4:  commit a String (e.g. "An island with two hills") and the function returns an array with words (e.g. ["An", "island", "with", "two", "hills"])
    private static native String[] returnJArrayfromJString(final String string);

    private static native String[] split(final String string);

    private static native String recover(final String[] string);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        //Load rust library
        System.loadLibrary("rust");

        // func 1
        int i = returni32(10);

        ((TextView) findViewById(R.id.func1Name)).setText("returni32 (5)");
        ((TextView) findViewById(R.id.func1Result)).setText("Result: " + i);


        // func 2
        boolean value = getFileStatus();

        ((TextView) findViewById(R.id.func2Name)).setText("getFileStatus()");
        ((TextView) findViewById(R.id.func2Result)).setText("Result: " + value);


        // func 3
        String string = returnJString("myString");

        ((TextView) findViewById(R.id.func3Name)).setText("returnJString(\"myString\")");
        ((TextView) findViewById(R.id.func3Result)).setText("Result: " + string);


        // func 4
        String[] split = split("1234");

        ((TextView) findViewById(R.id.func4Name)).setText("split(\"1234\")");
        ((TextView) findViewById(R.id.func4Result)).setText("Result: " + java.util.Arrays.toString(split));

        //func 5
        String[] params = {
                "1+9b276953e972d8421676e55ddea53ec37f687dc23443117adc65ac35be1f748d",
                "2+364ed2a7d2e5b0842cedcabbbd4a7d86fed0fb84688622f5b8cb586c7c3edab7",
                "3+d1763bfbbc5888c64364b0199befbc4a7e3979469cc93470953104a23a5e3d10"
        };
        String recover = recover(params);

        ((TextView) findViewById(R.id.func5Name)).setText("recover(\"" + java.util.Arrays.toString(params) + "\")");
        ((TextView) findViewById(R.id.func5Result)).setText("Result: " + recover);
    }
}
