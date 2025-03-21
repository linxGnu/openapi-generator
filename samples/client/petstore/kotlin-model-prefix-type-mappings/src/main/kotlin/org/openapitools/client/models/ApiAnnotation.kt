/**
 *
 * Please note:
 * This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit this file manually.
 *
 */

@file:Suppress(
    "ArrayInDataClass",
    "EnumEntryName",
    "RemoveRedundantQualifierName",
    "UnusedImport"
)

package org.openapitools.client.models


import com.google.gson.Gson
import com.google.gson.JsonElement
import com.google.gson.TypeAdapter
import com.google.gson.TypeAdapterFactory
import com.google.gson.reflect.TypeToken
import com.google.gson.stream.JsonReader
import com.google.gson.stream.JsonWriter
import com.google.gson.annotations.JsonAdapter
import java.io.IOException
import com.google.gson.annotations.SerializedName

/**
 * 
 *
 * @param id 
 */


data class ApiAnnotation (

    @SerializedName("id")
    val id: java.util.UUID? = null

) {


    class CustomTypeAdapterFactory : TypeAdapterFactory {
        override fun <T> create(gson: Gson, type: TypeToken<T>): TypeAdapter<T>? {
            if (!ApiAnnotation::class.java.isAssignableFrom(type.rawType)) {
              return null // this class only serializes 'ApiAnnotation' and its subtypes
            }
            val elementAdapter = gson.getAdapter(JsonElement::class.java)
            val thisAdapter = gson.getDelegateAdapter(this, TypeToken.get(ApiAnnotation::class.java))

            @Suppress("UNCHECKED_CAST")
            return object : TypeAdapter<ApiAnnotation>() {
                @Throws(IOException::class)
                override fun write(out: JsonWriter, value: ApiAnnotation) {
                    val obj = thisAdapter.toJsonTree(value).getAsJsonObject()
                    elementAdapter.write(out, obj)
                }

                @Throws(IOException::class)
                override fun read(jsonReader: JsonReader): ApiAnnotation  {
                    val jsonElement = elementAdapter.read(jsonReader)
                    validateJsonElement(jsonElement)
                    return thisAdapter.fromJsonTree(jsonElement)
                }
            }.nullSafe() as TypeAdapter<T>
        }
    }

    companion object {
        var openapiFields = HashSet<String>()
        var openapiRequiredFields = HashSet<String>()

        init {
            // a set of all properties/fields (JSON key names)
            openapiFields.add("id")

        }

       /**
        * Validates the JSON Element and throws an exception if issues found
        *
        * @param jsonElement JSON Element
        * @throws IOException if the JSON Element is invalid with respect to ApiAnnotation
        */
        @Throws(IOException::class)
        fun validateJsonElement(jsonElement: JsonElement?) {
            if (jsonElement == null) {
              require(openapiRequiredFields.isEmpty()) { // has required fields but JSON element is null
                String.format("The required field(s) %s in ApiAnnotation is not found in the empty JSON string", ApiAnnotation.openapiRequiredFields.toString())
              }
            }
            val jsonObj = jsonElement!!.getAsJsonObject()
            if (jsonObj["id"] != null && !jsonObj["id"].isJsonNull) {
              require(jsonObj.get("id").isJsonPrimitive) {
                String.format("Expected the field `id` to be a primitive type in the JSON string but got `%s`", jsonObj["id"].toString())
              }
            }
        }
    }

}

