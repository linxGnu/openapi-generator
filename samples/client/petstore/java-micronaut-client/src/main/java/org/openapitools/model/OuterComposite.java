/*
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

package org.openapitools.model;

import java.util.Objects;
import java.util.Arrays;
import java.math.BigDecimal;
import com.fasterxml.jackson.annotation.*;

import javax.validation.constraints.*;
import javax.validation.Valid;
import io.micronaut.core.annotation.*;
import javax.annotation.Generated;

/**
 * OuterComposite
 */
@JsonPropertyOrder({
  OuterComposite.JSON_PROPERTY_MY_NUMBER,
  OuterComposite.JSON_PROPERTY_MY_STRING,
  OuterComposite.JSON_PROPERTY_MY_BOOLEAN
})
@JsonTypeName("OuterComposite")
@Generated(value="org.openapitools.codegen.languages.JavaMicronautClientCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
@Introspected
public class OuterComposite {
    public static final String JSON_PROPERTY_MY_NUMBER = "my_number";
    private BigDecimal myNumber;

    public static final String JSON_PROPERTY_MY_STRING = "my_string";
    private String myString;

    public static final String JSON_PROPERTY_MY_BOOLEAN = "my_boolean";
    private Boolean myBoolean;

    public OuterComposite() {
    }
    public OuterComposite myNumber(BigDecimal myNumber) {
        this.myNumber = myNumber;
        return this;
    }

    /**
     * Get myNumber
     * @return myNumber
     */
    @Nullable
    @JsonProperty(JSON_PROPERTY_MY_NUMBER)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public BigDecimal getMyNumber() {
        return myNumber;
    }

    @JsonProperty(JSON_PROPERTY_MY_NUMBER)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setMyNumber(BigDecimal myNumber) {
        this.myNumber = myNumber;
    }

    public OuterComposite myString(String myString) {
        this.myString = myString;
        return this;
    }

    /**
     * Get myString
     * @return myString
     */
    @Nullable
    @JsonProperty(JSON_PROPERTY_MY_STRING)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public String getMyString() {
        return myString;
    }

    @JsonProperty(JSON_PROPERTY_MY_STRING)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setMyString(String myString) {
        this.myString = myString;
    }

    public OuterComposite myBoolean(Boolean myBoolean) {
        this.myBoolean = myBoolean;
        return this;
    }

    /**
     * Get myBoolean
     * @return myBoolean
     */
    @Nullable
    @JsonProperty(JSON_PROPERTY_MY_BOOLEAN)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public Boolean getMyBoolean() {
        return myBoolean;
    }

    @JsonProperty(JSON_PROPERTY_MY_BOOLEAN)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setMyBoolean(Boolean myBoolean) {
        this.myBoolean = myBoolean;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        OuterComposite outerComposite = (OuterComposite) o;
        return Objects.equals(this.myNumber, outerComposite.myNumber) &&
            Objects.equals(this.myString, outerComposite.myString) &&
            Objects.equals(this.myBoolean, outerComposite.myBoolean);
    }

    @Override
    public int hashCode() {
        return Objects.hash(myNumber, myString, myBoolean);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("class OuterComposite {\n");
        sb.append("    myNumber: ").append(toIndentedString(myNumber)).append("\n");
        sb.append("    myString: ").append(toIndentedString(myString)).append("\n");
        sb.append("    myBoolean: ").append(toIndentedString(myBoolean)).append("\n");
        sb.append("}");
        return sb.toString();
    }

    /**
     * Convert the given object to string with each line indented by 4 spaces
     * (except the first line).
     */
    private String toIndentedString(Object o) {
        if (o == null) {
            return "null";
        }
        return o.toString().replace("\n", "\n    ");
    }

}

